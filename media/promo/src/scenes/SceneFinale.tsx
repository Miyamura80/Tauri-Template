import type React from "react";
import {
  interpolate,
  spring,
  useCurrentFrame,
  useVideoConfig,
} from "remotion";
import { loadFont } from "@remotion/google-fonts/Inter";
import { COLORS } from "../shared/colors";

const { fontFamily } = loadFont("normal", {
  weights: ["700", "400"],
  subsets: ["latin"],
});

export const SceneFinale: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  // Everything scales down
  const scaleDown = spring({
    frame,
    fps,
    config: { damping: 200 },
    durationInFrames: 20,
  });
  const bgOpacity = interpolate(scaleDown, [0, 1], [1, 0.3]);

  // Badge pops up from bottom
  const badgePop = spring({
    frame,
    fps,
    delay: 10,
    config: { damping: 12, stiffness: 200 },
  });
  const badgeY = interpolate(badgePop, [0, 1], [520, 185]);
  const badgeScale = interpolate(badgePop, [0, 1], [0.5, 1]);

  // Subtitle fades in
  const subtitleOpacity = interpolate(frame, [30, 45], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  // Badge glow pulse
  const glowPulse = interpolate(
    Math.sin(frame * 0.1),
    [-1, 1],
    [0.3, 0.7],
  );

  return (
    <>
      {/* Dim previous content layer */}
      <div
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          height: "100%",
          background: `rgba(15, 13, 26, ${1 - bgOpacity})`,
        }}
      />

      {/* Badge */}
      <div
        style={{
          position: "absolute",
          left: "50%",
          top: badgeY,
          transform: `translate(-50%, -50%) scale(${badgeScale})`,
          background: COLORS.badgeBg,
          border: `2px solid ${COLORS.badgeBorder}`,
          borderRadius: 16,
          padding: "18px 40px",
          boxShadow: `0 0 ${30 * glowPulse}px ${10 * glowPulse}px rgba(124, 77, 255, 0.3)`,
          textAlign: "center" as const,
        }}
      >
        <div
          style={{
            fontFamily,
            fontWeight: 700,
            fontSize: 36,
            color: COLORS.textPrimary,
            lineHeight: 1.2,
          }}
        >
          Built & Tested
        </div>
        <div
          style={{
            fontFamily,
            fontWeight: 700,
            fontSize: 14,
            color: COLORS.checkGreen,
            marginTop: 6,
            letterSpacing: 2,
          }}
        >
          &#10003; &#10003; &#10003;
        </div>
      </div>

      {/* Subtitle */}
      <div
        style={{
          position: "absolute",
          left: "50%",
          top: 275,
          transform: "translateX(-50%)",
          opacity: subtitleOpacity,
          fontFamily,
          fontWeight: 400,
          fontSize: 18,
          color: COLORS.textSecondary,
          letterSpacing: 3,
        }}
      >
        TAURI-TEMPLATE
      </div>
    </>
  );
};
