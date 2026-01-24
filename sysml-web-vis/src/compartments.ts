import type { VisSpecCompartment, VisSpecViewMetadata } from "./vis-spec";

export type CompartmentMode = "textual" | "graphical" | "mixed";

export function resolveCompartmentMode(
  viewMetadata?: VisSpecViewMetadata,
): CompartmentMode {
  return viewMetadata?.compartmentMode ?? "mixed";
}

export function filterTextCompartments(
  compartments: VisSpecCompartment[] | undefined,
  mode: CompartmentMode,
): VisSpecCompartment[] {
  if (!compartments || mode === "graphical") {
    return [];
  }

  return compartments.filter((compartment) => compartment.kind === "text");
}

export function filterGraphicalCompartments(
  compartments: VisSpecCompartment[] | undefined,
  mode: CompartmentMode,
): VisSpecCompartment[] {
  if (!compartments || mode === "textual") {
    return [];
  }

  return compartments.filter(
    (compartment) => compartment.kind === "graphical",
  );
}
