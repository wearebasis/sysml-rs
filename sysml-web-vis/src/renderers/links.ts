import { dia } from "@joint/core";
import { VisSpecLink } from "../vis-spec";

export interface ParallelLinkInfo {
  offsetIndex: number;
  total: number;
}

export function buildParallelLinkIndex(
  links: VisSpecLink[],
): Map<string, ParallelLinkInfo> {
  const buckets = new Map<string, string[]>();

  links.forEach((link) => {
    const key = buildParallelKey(link);
    const bucket = buckets.get(key) ?? [];
    bucket.push(link.id);
    buckets.set(key, bucket);
  });

  const result = new Map<string, ParallelLinkInfo>();
  buckets.forEach((ids) => {
    if (ids.length <= 1) {
      result.set(ids[0], { offsetIndex: 0, total: 1 });
      return;
    }
    ids.forEach((id, index) => {
      result.set(id, {
        offsetIndex: index - (ids.length - 1) / 2,
        total: ids.length,
      });
    });
  });

  return result;
}

export function applyParallelLinkOffset(
  link: dia.Link,
  source: dia.Element,
  target: dia.Element,
  info: ParallelLinkInfo | undefined,
  offsetStep = 18,
): void {
  if (!info || info.total <= 1 || info.offsetIndex === 0) {
    return;
  }

  const sourceBox = source.getBBox();
  const targetBox = target.getBBox();
  const sourceCenter = {
    x: sourceBox.x + sourceBox.width / 2,
    y: sourceBox.y + sourceBox.height / 2,
  };
  const targetCenter = {
    x: targetBox.x + targetBox.width / 2,
    y: targetBox.y + targetBox.height / 2,
  };

  const dx = targetCenter.x - sourceCenter.x;
  const dy = targetCenter.y - sourceCenter.y;
  const length = Math.hypot(dx, dy) || 1;
  const offset = info.offsetIndex * offsetStep;

  const mid = {
    x: (sourceCenter.x + targetCenter.x) / 2,
    y: (sourceCenter.y + targetCenter.y) / 2,
  };

  const vertex = {
    x: mid.x + (-dy / length) * offset,
    y: mid.y + (dx / length) * offset,
  };

  link.vertices([vertex]);
}

function buildParallelKey(link: VisSpecLink): string {
  const sourceKey = `${link.source.nodeId}:${link.source.portId ?? ""}`;
  const targetKey = `${link.target.nodeId}:${link.target.portId ?? ""}`;
  if (sourceKey < targetKey) {
    return `${sourceKey}|${targetKey}`;
  }
  return `${targetKey}|${sourceKey}`;
}
