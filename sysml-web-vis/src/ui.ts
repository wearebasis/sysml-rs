import type { LayoutStrategy } from "./layout";
import type { VisSpec, VisSpecView } from "./vis-spec";

export type RouterStrategy = "default" | "avoid";

interface ControlOptions {
  viewSelect: HTMLSelectElement | null;
  layoutSelect: HTMLSelectElement | null;
  routerSelect: HTMLSelectElement | null;
  layoutAllToggle?: HTMLInputElement | null;
  viewOptions: VisSpecView[];
  layoutOptions: LayoutStrategy[];
  routerOptions: RouterStrategy[];
  initialView: VisSpecView;
  initialLayout: LayoutStrategy;
  initialRouter: RouterStrategy;
  initialLayoutAll?: boolean;
  resolveLayoutOptions: (view: VisSpecView) => LayoutStrategy[];
  resolveRouterOptions: (view: VisSpecView) => RouterStrategy[];
}

export function initializeControls(options: ControlOptions): void {
  const { viewSelect, layoutSelect, routerSelect, layoutAllToggle } = options;
  if (!viewSelect || !layoutSelect) {
    return;
  }

  options.viewOptions.forEach((view) => {
    const option = document.createElement("option");
    option.value = view;
    option.textContent = view;
    viewSelect.append(option);
  });

  options.layoutOptions.forEach((layout) => {
    const option = document.createElement("option");
    option.value = layout;
    option.textContent = layout;
    layoutSelect.append(option);
  });

  if (routerSelect) {
    options.routerOptions.forEach((router) => {
      const option = document.createElement("option");
      option.value = router;
      option.textContent = router;
      routerSelect.append(option);
    });
    routerSelect.value = options.initialRouter;
  }

  viewSelect.value = options.initialView;
  layoutSelect.value = options.initialLayout;
  if (layoutAllToggle) {
    layoutAllToggle.checked = options.initialLayoutAll ?? false;
  }

  const layoutLabel = document.querySelector(
    'label[for="layoutSelect"]',
  ) as HTMLElement | null;
  const routerLabel = document.querySelector(
    'label[for="routerSelect"]',
  ) as HTMLElement | null;
  const layoutAllLabel =
    (layoutAllToggle?.closest("label") as HTMLElement | null) ?? null;

  const showLayout = options.layoutOptions.length > 1;
  const showRouter = options.routerOptions.length > 1;

  toggleControl(layoutLabel, showLayout);
  toggleControl(layoutSelect, showLayout);
  toggleControl(layoutAllLabel, showLayout);
  toggleControl(routerLabel, showRouter);
  toggleControl(routerSelect, showRouter);

  const updateQuery = () => {
    const next = new URLSearchParams(window.location.search);
    const selectedView = viewSelect.value as VisSpecView;
    const layoutConfig = resolveLayoutParams(
      selectedView,
      layoutSelect,
      options.resolveLayoutOptions,
    );
    const routerConfig = resolveRouterParams(
      selectedView,
      routerSelect,
      options.resolveRouterOptions,
    );
    next.set("view", selectedView);
    applyLayoutParams(next, layoutConfig, layoutAllToggle);
    applyRouterParams(next, routerConfig);
    window.location.search = next.toString();
  };

  const updateViewQuery = () => {
    const next = new URLSearchParams(window.location.search);
    const nextView = viewSelect.value as VisSpecView;
    const layoutConfig = resolveLayoutParams(
      nextView,
      layoutSelect,
      options.resolveLayoutOptions,
    );
    const routerConfig = resolveRouterParams(
      nextView,
      routerSelect,
      options.resolveRouterOptions,
    );
    next.delete("fixture");
    next.set("view", nextView);
    applyLayoutParams(next, layoutConfig, layoutAllToggle);
    applyRouterParams(next, routerConfig);
    window.location.search = next.toString();
  };

  viewSelect.addEventListener("change", updateViewQuery);
  if (showLayout) {
    layoutSelect.addEventListener("change", updateQuery);
    layoutAllToggle?.addEventListener("change", updateQuery);
  }
  if (showRouter) {
    routerSelect?.addEventListener("change", updateQuery);
  }
}

function toggleControl(
  element: HTMLElement | null | undefined,
  visible: boolean,
): void {
  if (!element) {
    return;
  }
  element.hidden = !visible;
  if (element instanceof HTMLSelectElement) {
    element.disabled = !visible;
  }
  if (element instanceof HTMLInputElement) {
    element.disabled = !visible;
  }
}

type LayoutParamConfig = {
  enabled: boolean;
  value: LayoutStrategy | null;
};

type RouterParamConfig = {
  enabled: boolean;
  value: RouterStrategy | null;
};

function resolveLayoutParams(
  view: VisSpecView,
  layoutSelect: HTMLSelectElement | null,
  resolver: (view: VisSpecView) => LayoutStrategy[],
): LayoutParamConfig {
  const options = resolver(view);
  const nextValue =
    options.length > 1 && layoutSelect
      ? options.includes(layoutSelect.value as LayoutStrategy)
        ? (layoutSelect.value as LayoutStrategy)
        : options[0]
      : (options[0] ?? null);
  return {
    enabled: options.length > 1,
    value: options.length > 0 ? (nextValue ?? options[0]) : null,
  };
}

function resolveRouterParams(
  view: VisSpecView,
  routerSelect: HTMLSelectElement | null,
  resolver: (view: VisSpecView) => RouterStrategy[],
): RouterParamConfig {
  const options = resolver(view);
  const nextValue =
    options.length > 1 && routerSelect
      ? options.includes(routerSelect.value as RouterStrategy)
        ? (routerSelect.value as RouterStrategy)
        : options[0]
      : (options[0] ?? null);
  return {
    enabled: options.length > 1,
    value: options.length > 0 ? (nextValue ?? options[0]) : null,
  };
}

function applyLayoutParams(
  params: URLSearchParams,
  config: LayoutParamConfig,
  layoutAllToggle?: HTMLInputElement | null,
): void {
  if (config.enabled && config.value) {
    params.set("layout", config.value);
    if (layoutAllToggle && layoutAllToggle.checked) {
      params.set("layoutAll", "1");
    } else {
      params.delete("layoutAll");
    }
  } else {
    params.delete("layout");
    params.delete("layoutAll");
  }
}

function applyRouterParams(
  params: URLSearchParams,
  config: RouterParamConfig,
): void {
  if (config.enabled && config.value) {
    params.set("router", config.value);
  } else {
    params.delete("router");
  }
}

export function updateDiagramHeader(spec: VisSpec): void {
  const header = document.getElementById("diagramHeader");
  const title = document.getElementById("diagramTitle");
  const subtitle = document.getElementById("diagramSubtitle");
  const meta = document.getElementById("diagramMeta");

  if (!header || !title || !subtitle || !meta) {
    return;
  }

  const metadata = spec.viewMetadata;
  const titleText = metadata?.title ?? spec.view;
  const subtitleParts = [
    metadata?.subtitle,
    metadata?.viewpoint ? `Viewpoint: ${metadata.viewpoint}` : undefined,
    metadata?.subject ? `Subject: ${metadata.subject}` : undefined,
  ].filter(Boolean);

  title.textContent = titleText;
  subtitle.textContent = subtitleParts.join(" • ");

  const metaItems: string[] = [];
  if (metadata?.compartmentMode) {
    metaItems.push(`Compartments: ${metadata.compartmentMode}`);
  }
  if (metadata?.description) {
    metaItems.push(metadata.description);
  }
  if (metaItems.length === 0) {
    meta.textContent = "";
    return;
  }

  meta.innerHTML = "";
  metaItems.forEach((item) => {
    const span = document.createElement("span");
    span.textContent = item;
    meta.append(span);
  });
}
