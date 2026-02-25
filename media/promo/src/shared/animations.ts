import { interpolate, spring } from "remotion";

type SpringOpts = {
  frame: number;
  fps: number;
  delay?: number;
  config?: { damping?: number; stiffness?: number; mass?: number };
  durationInFrames?: number;
};

export const smoothSpring = (opts: SpringOpts) =>
  spring({ ...opts, config: { damping: 200, ...opts.config } });

export const bouncySpring = (opts: SpringOpts) =>
  spring({ ...opts, config: { damping: 8, ...opts.config } });

export const snappySpring = (opts: SpringOpts) =>
  spring({
    ...opts,
    config: { damping: 20, stiffness: 200, ...opts.config },
  });

export const slideIn = (
  frame: number,
  fps: number,
  from: number,
  to: number,
  delay = 0,
) => {
  const progress = smoothSpring({ frame, fps, delay });
  return interpolate(progress, [0, 1], [from, to]);
};

export const fadeIn = (
  frame: number,
  startFrame: number,
  duration: number,
) => {
  return interpolate(frame, [startFrame, startFrame + duration], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
};

export const bob = (frame: number, amplitude = 4, speed = 0.08) => {
  return Math.sin(frame * speed) * amplitude;
};
