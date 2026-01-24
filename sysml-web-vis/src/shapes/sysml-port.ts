const basePortBody = {
  width: 12,
  height: 12,
  x: -6,
  y: -6,
  rx: 1.5,
  ry: 1.5,
  magnet: true,
  fill: "var(--sysml-node-fill)",
  stroke: "var(--sysml-node-stroke)",
  strokeWidth: 1.4,
};

const basePortLabel = {
  fontSize: 10,
  fill: "var(--sysml-muted)",
  fontFamily: "Helvetica Neue, Helvetica, Arial, sans-serif",
  textVerticalAnchor: "middle",
  textAnchor: "middle",
};

const basePortDecoration = {
  stroke: "var(--sysml-node-stroke)",
  strokeWidth: 1.4,
  display: "none",
};

export const SysmlPortGroups = {
  left: {
    position: { name: "left" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: -6,
        y1: 0,
        x2: -16,
        y2: 0,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
  right: {
    position: { name: "right" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: 6,
        y1: 0,
        x2: 16,
        y2: 0,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
  top: {
    position: { name: "top" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: 0,
        y1: -6,
        x2: 0,
        y2: -16,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
  bottom: {
    position: { name: "bottom" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: 0,
        y1: 6,
        x2: 0,
        y2: 16,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
  in: {
    position: { name: "left" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: -6,
        y1: 0,
        x2: -16,
        y2: 0,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
  out: {
    position: { name: "right" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: 6,
        y1: 0,
        x2: 16,
        y2: 0,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
  inout: {
    position: { name: "bottom" },
    attrs: {
      portBody: basePortBody,
      portDecoration: {
        ...basePortDecoration,
        x1: 0,
        y1: 6,
        x2: 0,
        y2: 16,
      },
      labelText: {
        ...basePortLabel,
      },
    },
  },
};
