import type React from "react";
import { AbsoluteFill, useCurrentFrame } from "remotion";
import { COLORS } from "./colors";

export const Background: React.FC = () => {
  const frame = useCurrentFrame();
  const hueShift = frame * 0.15;

  return (
    <AbsoluteFill>
      <div
        style={{
          width: "100%",
          height: "100%",
          background: `linear-gradient(${135 + hueShift}deg, ${COLORS.bgDark} 0%, ${COLORS.bgMid} 50%, ${COLORS.bgAccent} 100%)`,
        }}
      />
      {/* Subtle grid overlay */}
      <div
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          height: "100%",
          backgroundImage: `radial-gradient(circle, rgba(124,77,255,0.06) 1px, transparent 1px)`,
          backgroundSize: "30px 30px",
        }}
      />
    </AbsoluteFill>
  );
};
