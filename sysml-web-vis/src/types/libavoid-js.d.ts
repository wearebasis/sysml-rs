declare module "libavoid-js" {
  export const AvoidLib: {
    load: (wasmPath?: string) => Promise<void>;
    getInstance: () => unknown;
  };
}
