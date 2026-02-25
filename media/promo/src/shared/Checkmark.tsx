import type React from "react";
import { spring, useCurrentFrame, useVideoConfig } from "remotion";
import { COLORS } from "./colors";

type Props = {
  x: number;
  y: number;
  delay?: number;
  size?: number;
};

export const Checkmark: React.FC<Props> = ({
  x,
  y,
  delay = 0,
  size = 28,
}) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const progress = spring({
    frame,
    fps,
    delay,
    config: { damping: 12, stiffness: 200 },
  });

  const scale = progress;
  const pathLength = progress * 24;

  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 28 28"
      style={{
        position: "absolute",
        left: x - size / 2,
        top: y - size / 2,
        transform: `scale(${scale})`,
      }}
    >
      <circle cx={14} cy={14} r={13} fill={COLORS.checkGreen} opacity={0.2} />
      <circle
        cx={14}
        cy={14}
        r={13}
        fill="none"
        stroke={COLORS.checkGreen}
        strokeWidth={2}
      />
      <path
        d="M8 14 L12 18 L20 10"
        fill="none"
        stroke={COLORS.checkGreen}
        strokeWidth={2.5}
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeDasharray={24}
        strokeDashoffset={24 - pathLength}
      />
    </svg>
  );
};
