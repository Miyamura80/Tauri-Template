import type React from "react";
import { useCurrentFrame } from "remotion";
import { bob } from "../shared/animations";
import { COLORS } from "../shared/colors";

type Props = {
  x: number;
  y: number;
  scale?: number;
  clawAngle?: number;
  bobOffset?: number;
};

export const Crab: React.FC<Props> = ({
  x,
  y,
  scale = 1,
  clawAngle = 0,
  bobOffset,
}) => {
  const frame = useCurrentFrame();
  const bobY = bobOffset ?? bob(frame, 3, 0.1);

  return (
    <svg
      width={120}
      height={100}
      viewBox="0 0 120 100"
      style={{
        position: "absolute",
        left: x - 60,
        top: y - 50 + bobY,
        transform: `scale(${scale})`,
        transformOrigin: "center center",
      }}
    >
      {/* Legs - 3 per side */}
      {[-1, 1].map((side) =>
        [0, 1, 2].map((i) => (
          <line
            key={`leg-${side}-${i}`}
            x1={60 + side * 22}
            y1={58 + i * 8}
            x2={60 + side * (42 + i * 5)}
            y2={72 + i * 6}
            stroke={COLORS.crabDark}
            strokeWidth={3}
            strokeLinecap="round"
          />
        )),
      )}

      {/* Left claw */}
      <g
        transform={`rotate(${-clawAngle}, 25, 40)`}
        style={{ transformOrigin: "25px 40px" }}
      >
        <ellipse cx={18} cy={30} rx={12} ry={8} fill={COLORS.crabBody} />
        <path
          d="M10 26 Q6 20 12 18 Q18 16 16 24"
          fill={COLORS.crabLight}
          stroke={COLORS.crabDark}
          strokeWidth={1}
        />
        <path
          d="M20 26 Q24 20 20 18 Q14 16 16 24"
          fill={COLORS.crabLight}
          stroke={COLORS.crabDark}
          strokeWidth={1}
        />
      </g>

      {/* Right claw */}
      <g
        transform={`rotate(${clawAngle}, 95, 40)`}
        style={{ transformOrigin: "95px 40px" }}
      >
        <ellipse cx={102} cy={30} rx={12} ry={8} fill={COLORS.crabBody} />
        <path
          d="M96 26 Q92 20 98 18 Q104 16 102 24"
          fill={COLORS.crabLight}
          stroke={COLORS.crabDark}
          strokeWidth={1}
        />
        <path
          d="M106 26 Q110 20 106 18 Q100 16 102 24"
          fill={COLORS.crabLight}
          stroke={COLORS.crabDark}
          strokeWidth={1}
        />
      </g>

      {/* Body */}
      <ellipse cx={60} cy={55} rx={30} ry={22} fill={COLORS.crabBody} />
      <ellipse cx={60} cy={52} rx={26} ry={18} fill={COLORS.crabLight} opacity={0.3} />

      {/* Eyes */}
      <circle cx={48} cy={42} r={8} fill={COLORS.crabEye} />
      <circle cx={72} cy={42} r={8} fill={COLORS.crabEye} />
      <circle cx={50} cy={41} r={4} fill={COLORS.crabPupil} />
      <circle cx={74} cy={41} r={4} fill={COLORS.crabPupil} />
      {/* Eye shine */}
      <circle cx={52} cy={39} r={1.5} fill={COLORS.crabEye} />
      <circle cx={76} cy={39} r={1.5} fill={COLORS.crabEye} />

      {/* Mouth - happy curve */}
      <path
        d="M52 58 Q60 64 68 58"
        fill="none"
        stroke={COLORS.crabDark}
        strokeWidth={2}
        strokeLinecap="round"
      />
    </svg>
  );
};
