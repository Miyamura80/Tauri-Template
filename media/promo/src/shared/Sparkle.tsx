import type React from "react";
import { interpolate, useCurrentFrame } from "remotion";
import { COLORS } from "./colors";

type Props = {
  x: number;
  y: number;
  delay?: number;
  size?: number;
};

export const Sparkle: React.FC<Props> = ({
  x,
  y,
  delay = 0,
  size = 12,
}) => {
  const frame = useCurrentFrame();
  const localFrame = frame - delay;

  if (localFrame < 0) return null;

  const lifespan = 30;
  const progress = Math.min(localFrame / lifespan, 1);

  const scale = interpolate(progress, [0, 0.3, 1], [0, 1.2, 0], {
    extrapolateRight: "clamp",
  });
  const opacity = interpolate(progress, [0, 0.2, 0.8, 1], [0, 1, 1, 0], {
    extrapolateRight: "clamp",
  });
  const rotation = localFrame * 3;

  if (opacity <= 0) return null;

  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      style={{
        position: "absolute",
        left: x - size / 2,
        top: y - size / 2,
        opacity,
        transform: `scale(${scale}) rotate(${rotation}deg)`,
      }}
    >
      <path
        d="M12 0 L14 10 L24 12 L14 14 L12 24 L10 14 L0 12 L10 10 Z"
        fill={COLORS.sparkle}
      />
    </svg>
  );
};
