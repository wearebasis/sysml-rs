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

  const updateQuery = () => {
    const next = new URLSearchParams(window.location.search);
    next.set("view", viewSelect.value);
    next.set("layout", layoutSelect.value);
    if (routerSelect) {
      next.set("router", routerSelect.value);
    }
    if (layoutAllToggle) {
      if (layoutAllToggle.checked) {
        next.set("layoutAll", "1");
      } else {
        next.delete("layoutAll");
      }
    }
    window.location.search = next.toString();
  };

  const updateViewQuery = () => {
    const next = new URLSearchParams(window.location.search);
    next.delete("fixture");
    next.set("view", viewSelect.value);
    next.set("layout", layoutSelect.value);
    if (routerSelect) {
      next.set("router", routerSelect.value);
    }
    if (layoutAllToggle) {
      if (layoutAllToggle.checked) {
        next.set("layoutAll", "1");
      } else {
        next.delete("layoutAll");
      }
    }
    window.location.search = next.toString();
  };

  viewSelect.addEventListener("change", updateViewQuery);
  layoutSelect.addEventListener("change", updateQuery);
  layoutAllToggle?.addEventListener("change", updateQuery);
  routerSelect?.addEventListener("change", updateQuery);
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
