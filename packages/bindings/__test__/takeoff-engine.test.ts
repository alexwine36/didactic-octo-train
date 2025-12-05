import { test, expect, vi, describe } from 'vitest'
import { TakeoffEngine, Transform } from '../index.js'

describe('TakeoffEngine', () => {
  test('TakeoffEngine - coordinate transformation', () => {
    const engine = new TakeoffEngine(
      {
        scale: 1.0,
        offsetX: 0.0,
        offsetY: 0.0,
      },
      null,
    )

    const transformCallback = (_err: Error | null, arg: Transform) => {
      // t.deepEqual(arg, { scale: 2.0, offsetX: 0.0, offsetY: 0.0 })
      console.log(arg)
    }

    engine.addTransformCallback(transformCallback)

    expect(engine.screenToWorld({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 })
    expect(engine.worldToScreen({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 })
    expect(engine.zoomAroundPoint({ x: 0, y: 0 }, 1.0)).toEqual({ scale: 1.0, offsetX: 0.0, offsetY: 0.0 })
    expect(engine.pan(1.0, 1.0)).toEqual({ scale: 1.0, offsetX: 1.0, offsetY: 1.0 })
    expect(engine.resetTransform()).toEqual({ scale: 1.0, offsetX: 0.0, offsetY: 0.0 })
    expect(engine.fitToViewport(100, 100, 100, 100)).toEqual({ scale: 1.0, offsetX: 0.0, offsetY: 0.0 })
  })

  test('TakeoffEngine - transform callback', async () => {
    const engine = new TakeoffEngine(
      {
        scale: 1.0,
        offsetX: 0.0,
        offsetY: 0.0,
      },
      null,
    )

    const testCallback = vi.fn((_err: Error | null, arg: Transform) => {
      console.log('transformCallback: ', arg)
    })

    engine.addTransformCallback(testCallback)
    engine.pan(1.0, 1.0)
    await new Promise((resolve) => setTimeout(resolve, 250))

    expect(testCallback).toHaveBeenCalled()
  })
})
