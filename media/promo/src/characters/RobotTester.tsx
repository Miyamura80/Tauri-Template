import type React from "react";
import { interpolate, useCurrentFrame } from "remotion";
import { COLORS } from "../shared/colors";

type Props = {
  x: number;
  y: number;
  armRaise?: number;
  antennaGlow?: number;
  variant?: number;
};

export const RobotTester: React.FC<Props> = ({
  x,
  y,
  armRaise = 0,
  antennaGlow = 0,
  variant = 0,
}) => {
  const frame = useCurrentFrame();
  const blink = Math.sin(frame * 0.15 + variant * 2) > 0.92 ? 0.3 : 1;
  const antennaFlicker =
    interpolate(Math.sin(frame * 0.3 + variant), [-1, 1], [0.4, 1]) *
    antennaGlow;

  const armAngle = interpolate(armRaise, [0, 1], [0, -45]);

  // Slightly different hue per variant
  const bodyColor =
    variant === 0
      ? COLORS.robotBody
      : variant === 1
        ? COLORS.robotLight
        : COLORS.robotDark;

  return (
    <svg
      width={50}
      height={70}
      viewBox="0 0 50 70"
      style={{
        position: "absolute",
        left: x - 25,
        top: y - 35,
      }}
    >
      {/* Antenna */}
      <line
        x1={25}
        y1={15}
        x2={25}
        y2={4}
        stroke={COLORS.robotDark}
        strokeWidth={2}
      />
      <circle
        cx={25}
        cy={3}
        r={3}
        fill={COLORS.robotAntenna}
        opacity={antennaFlicker}
      />

      {/* Head */}
      <rect x={15} y={14} width={20} height={16} rx={3} fill={bodyColor} />
      {/* LED eyes */}
      <circle cx={21} cy={22} r={2.5} fill={COLORS.robotEye} opacity={blink} />
      <circle cx={29} cy={22} r={2.5} fill={COLORS.robotEye} opacity={blink} />

      {/* Body */}
      <rect x={12} y={32} width={26} height={22} rx={3} fill={bodyColor} />
      {/* Chest panel */}
      <rect x={17} y={36} width={16} height={8} rx={2} fill={COLORS.robotDark} opacity={0.3} />
      {/* Chest light */}
      <circle cx={25} cy={40} r={2} fill={COLORS.robotEye} opacity={antennaGlow * 0.7} />

      {/* Left arm */}
      <g transform={`rotate(${armAngle}, 12, 34)`}>
        <rect x={4} y={34} width={7} height={16} rx={3} fill={bodyColor} />
      </g>

      {/* Right arm */}
      <g transform={`rotate(${-armAngle}, 38, 34)`}>
        <rect x={39} y={34} width={7} height={16} rx={3} fill={bodyColor} />
      </g>

      {/* Legs */}
      <rect x={16} y={55} width={7} height={12} rx={2} fill={COLORS.robotDark} />
      <rect x={27} y={55} width={7} height={12} rx={2} fill={COLORS.robotDark} />
    </svg>
  );
};
