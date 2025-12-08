import { TakeoffEngine } from "local-bindings";
import { useEffect, useRef } from "react";

import { useTakeoffEngine } from "./use-takeoff-engine";
import { Button } from "./components/ui/button";
import { cn } from "./lib/utils";
export function Canvas() {
    const { engine } = useTakeoffEngine();
    const canvas = useRef<HTMLCanvasElement>(null);
    // const drawingEngine = useRef<DrawingEngine>(null);
    // useEffect(() => {
    //     if (canvas.current) {
    //         console.log("Canvas element found: ", canvas.current);
    //         drawingEngine.current = new DrawingEngine("canvas");
    //         drawingEngine.current.initializeCanvas(canvas.current);
    //     }
    // }, [canvas]);


    const handlePan = () => {
        engine.pan(1.0, 1.0);
    }
    console.group("Engine");
    console.log("screenToWorld: ", engine.screenToWorld({ x: 0, y: 0 }));
    console.log("worldToScreen: ", engine.worldToScreen({ x: 0, y: 0 }));
    console.groupEnd();
    return (
        <div
            className={cn('relative h-full w-full bg-muted',)}
        >
            <canvas ref={canvas} id="canvas" width={100} height={100} />
            <Button onClick={handlePan}>Pan</Button>
        </div>
    );
}