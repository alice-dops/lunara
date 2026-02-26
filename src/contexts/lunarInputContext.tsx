import React, { createContext, useContext, useEffect, useMemo, useRef } from "react";
import { listen } from "@tauri-apps/api/event";

export type NavDir = "up" | "down" | "left" | "right";
export type ActionBtn = "A" | "B" | "X" | "Y" | "C" | "Z";

type LunaraInputPayload = { type: "Dir", value: NavDir | "none" } | { type: "Btn", value: ActionBtn }

type DirectionHandler = (dir: NavDir) => void;
type ActionHandler = (btn: ActionBtn) => void;

type Handlers = {
  onDirection?: DirectionHandler;
  onAction?: ActionHandler;
};

type InputBus = {
  subscribe(handlers: Handlers): () => void;
};

const LunaraInputContext = createContext<InputBus | null>(null);

function isNavDir(v: string): v is NavDir {
  return v === "up" || v === "down" || v === "left" || v === "right";
}

function isActionBtn(v: string): v is ActionBtn {
  return v === "A" || v === "B" || v === "X" || v === "Y" || v === "C" || v === "Z";
}

export function LunaraInputProvider({ children }: { children: React.ReactNode }) {
  // store subscribers in a ref so we don't re-render on subscribe/unsubscribe
  const subsRef = useRef(new Map<number, Handlers>());
  const nextIdRef = useRef(1);

  const bus = useMemo<InputBus>(() => {
    return {
      subscribe(handlers: Handlers) {
        const id = nextIdRef.current++;
        subsRef.current.set(id, handlers);
        return () => {
          subsRef.current.delete(id);
        };
      },
    };
  }, []);

  useEffect(() => {
    let disposed = false;
    let unlisten: null | (() => void) = null;

    const start = async () => {
      const u = await listen<LunaraInputPayload>("lunara://input", (event) => {
        const payload = event.payload;
        const subs = Array.from(subsRef.current.values());

        if (payload.type === "Dir") {
          const dir = payload.value;
          if (dir === "none") return;
          if (!isNavDir(dir)) return;

          for (const h of subs) h.onDirection?.(dir);
          return;
        }

        if (payload.type == "Btn") {
          const btn = payload.value;
          if (!isActionBtn(btn)) return;

          for (const h of subs) h.onAction?.(btn);
        }

      });

      // If React already unmounted (StrictMode), immediately clean it
      if (disposed) {
        u();
        return;
      }

      unlisten = u;
    };

    start();

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  // useEffect(() => {
  //   let unlisten: null | (() => void) = null;
  //
  //   (async () => {
  //     unlisten = await listen<LunaraInputPayload>("lunara://input", (event) => {
  //       const payload = event.payload;
  //       console.log(event)
  //
  //       // Snapshot subscribers at time of event
  //       const subs = Array.from(subsRef.current.values());
  //
  //       if (payload.type === "Dir") {
  //         const dir = payload.value;
  //         if (dir === "none") return;
  //         if (!isNavDir(dir)) return;
  //
  //         for (const h of subs) h.onDirection?.(dir);
  //         return;
  //       }
  //
  //       if (payload.type == "Btn") {
  //         const btn = payload.value;
  //         if (!isActionBtn(btn)) return;
  //
  //         for (const h of subs) h.onAction?.(btn);
  //       }
  //     });
  //   })();
  //
  // return () => {
  //   unlisten?.();
  // };
  // }, []);

  return <LunaraInputContext.Provider value={bus}>{children}</LunaraInputContext.Provider>;
}

export function useLunaraInput(handlers: Handlers) {
  const ctx = useContext(LunaraInputContext);
  if (!ctx) {
    throw new Error("useLunaraInput must be used inside <LunaraInputProvider>.");
  }

  // Keep latest handlers without resubscribing every render
  const handlersRef = useRef<Handlers>(handlers);
  useEffect(() => {
    handlersRef.current = handlers;
  }, [handlers.onAction, handlers.onDirection]);

  useEffect(() => {
    // subscribe once; delegate to latest ref on calls
    return ctx.subscribe({
      onDirection: (d) => handlersRef.current.onDirection?.(d),
      onAction: (b) => handlersRef.current.onAction?.(b),
    });
  }, [ctx]);
}
