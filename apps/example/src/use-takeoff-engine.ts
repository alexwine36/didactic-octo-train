import { useEffect, useRef, useState } from "react";
import { TakeoffEngine, type Transform } from "local-bindings";

export function useTakeoffEngine() {
    const engine = useRef<TakeoffEngine>(new TakeoffEngine({
        scale: 1.0,
        offsetX: 0.0,
        offsetY: 0.0,
    }));


    const onTransformChange = (_err: Error | null, transform: Transform) => {
        console.group("Transform Change");
        console.log("transform: ", transform);
        console.log("screenToWorld: ", engine.current.screenToWorld({ x: 0, y: 0 }));
        console.log("worldToScreen: ", engine.current.worldToScreen({ x: 0, y: 0 }));
        console.groupEnd();

    }
    engine.current.addTransformCallback(onTransformChange);
    return {
        engine: engine.current,
    }
}