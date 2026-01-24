import { dia } from "@joint/core";
import { CONTAINER_PADDING } from "./config/constants";

export type AutoFitPadding = {
  left: number;
  right: number;
  top: number;
  bottom: number;
};

export interface AutoFitOptions {
  padding?: AutoFitPadding;
  minRect?: { width: number; height: number };
  filter?: (cell: dia.Cell) => boolean;
  sysmlAutoFit?: boolean;
}

export function fitElementToChildren(
  element: dia.Element,
  options: AutoFitOptions = {},
): void {
  const padding = options.padding ?? CONTAINER_PADDING;
  const filter = options.filter ?? ((cell: dia.Cell) => cell.isElement());

  element.fitToChildren({
    padding,
    minRect: options.minRect,
    filter,
    sysmlAutoFit: options.sysmlAutoFit ?? true,
  });
}
