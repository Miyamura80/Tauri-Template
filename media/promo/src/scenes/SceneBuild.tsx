import type React from "react";
import {
  interpolate,
  spring,
  useCurrentFrame,
  useVideoConfig,
} from "remotion";
import { Bun } from "../characters/Bun";
import { Crab } from "../characters/Crab";
import { DesktopApp } from "../characters/DesktopApp";
import { bob } from "../shared/animations";
import { Sparkle } from "../shared/Sparkle";

export const SceneBuild: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  // Characters shift to the left
  const shiftProgress = spring({
    frame,
    fps,
    config: { damping: 200 },
    durationInFrames: 35,
  });
  const crabX = interpolate(shiftProgress, [0, 1], [320, 180]);
  const bunX = interpolate(shiftProgress, [0, 1], [480, 300]);

  // Claw animate (building gesture)
  const clawAngle =
    frame > 20
      ? interpolate(
          Math.sin((frame - 20) * 0.15),
          [-1, 1],
          [-5, 20],
        )
      : 0;

  // Desktop app materializes in stages
  const titleBarProgress = spring({
    frame,
    fps,
    delay: 25,
    config: { damping: 15, stiffness: 200 },
  });

  const bodyProgress = spring({
    frame,
    fps,
    delay: 40,
    config: { damping: 200 },
    durationInFrames: 30,
  });

  const contentProgress = spring({
    frame,
    fps,
    delay: 60,
    config: { damping: 200 },
    durationInFrames: 25,
  });

  const glowProgress = interpolate(frame, [70, 90], [0, 0.6], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  const bobY = bob(frame, 2, 0.1);

  // Build sparkles
  const buildSparkles = frame > 30;

  return (
    <>
      <Crab
        x={crabX}
        y={250 + bobY}
        clawAngle={clawAngle}
        scale={0.9}
      />
      <Bun
        x={bunX}
        y={250 + bobY}
        steamOpacity={0.5}
        scale={0.9}
      />

      <DesktopApp
        x={550}
        y={225}
        titleBarScale={titleBarProgress}
        bodyHeight={bodyProgress}
        contentOpacity={contentProgress}
        borderGlow={glowProgress}
      />

      {/* Build sparkles around the app */}
      {buildSparkles && (
        <>
          <Sparkle x={480} y={170} delay={35} size={10} />
          <Sparkle x={620} y={180} delay={42} size={12} />
          <Sparkle x={510} y={290} delay={50} size={9} />
          <Sparkle x={600} y={260} delay={55} size={11} />
          <Sparkle x={460} y={240} delay={60} size={8} />
          <Sparkle x={640} y={220} delay={68} size={10} />
        </>
      )}
    </>
  );
};
