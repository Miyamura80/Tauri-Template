import type React from "react";
import { interpolate, useCurrentFrame } from "remotion";
import { bob } from "../shared/animations";
import { COLORS } from "../shared/colors";

type Props = {
  x: number;
  y: number;
  scale?: number;
  bobOffset?: number;
  steamOpacity?: number;
};

export const Bun: React.FC<Props> = ({
  x,
  y,
  scale = 1,
  bobOffset,
  steamOpacity = 0,
}) => {
  const frame = useCurrentFrame();
  const bobY = bobOffset ?? bob(frame, 3, 0.12);

  // Steam wisp animation
  const steamY1 = interpolate(frame % 40, [0, 40], [0, -18]);
  const steamY2 = interpolate((frame + 13) % 40, [0, 40], [0, -18]);

  return (
    <svg
      width={100}
      height={100}
      viewBox="0 0 100 100"
      style={{
        position: "absolute",
        left: x - 50,
        top: y - 50 + bobY,
        transform: `scale(${scale})`,
        transformOrigin: "center center",
      }}
    >
      {/* Steam wisps */}
      {steamOpacity > 0 && (
        <g opacity={steamOpacity}>
          <path
            d={`M38 ${28 + steamY1} Q42 ${22 + steamY1} 38 ${16 + steamY1}`}
            fill="none"
            stroke="white"
            strokeWidth={2}
            strokeLinecap="round"
            opacity={0.4}
          />
          <path
            d={`M58 ${26 + steamY2} Q62 ${20 + steamY2} 58 ${14 + steamY2}`}
            fill="none"
            stroke="white"
            strokeWidth={2}
            strokeLinecap="round"
            opacity={0.3}
          />
        </g>
      )}

      {/* Body dome */}
      <ellipse cx={50} cy={58} rx={32} ry={26} fill={COLORS.bunBody} />

      {/* Top pleated folds (zigzag) */}
      <path
        d="M22 48 L28 38 L34 46 L40 36 L46 46 L52 36 L58 46 L64 36 L70 46 L76 38 L78 48"
        fill={COLORS.bunFold}
        stroke={COLORS.bunBody}
        strokeWidth={1}
      />

      {/* Highlight on dome */}
      <ellipse cx={42} cy={48} rx={14} ry={8} fill={COLORS.bunLight} opacity={0.4} />

      {/* Eyes */}
      <circle cx={40} cy={56} r={3.5} fill={COLORS.bunEye} />
      <circle cx={60} cy={56} r={3.5} fill={COLORS.bunEye} />
      {/* Eye shine */}
      <circle cx={41.5} cy={54.5} r={1.2} fill="white" />
      <circle cx={61.5} cy={54.5} r={1.2} fill="white" />

      {/* Smile */}
      <path
        d="M42 65 Q50 72 58 65"
        fill="none"
        stroke={COLORS.bunSmile}
        strokeWidth={2}
        strokeLinecap="round"
      />

      {/* Bottom plate line */}
      <ellipse
        cx={50}
        cy={78}
        rx={28}
        ry={4}
        fill="none"
        stroke={COLORS.bunFold}
        strokeWidth={1.5}
      />
    </svg>
  );
};
