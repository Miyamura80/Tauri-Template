import type React from "react";
import {
  interpolate,
  spring,
  useCurrentFrame,
  useVideoConfig,
} from "remotion";
import { Bun } from "../characters/Bun";
import { Crab } from "../characters/Crab";
import { bob } from "../shared/animations";
import { Sparkle } from "../shared/Sparkle";

export const SceneEntrance: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  // Crab slides in from left
  const crabEntrance = spring({
    frame,
    fps,
    config: { damping: 200 },
    durationInFrames: 40,
  });
  const crabX = interpolate(crabEntrance, [0, 1], [-80, 320]);

  // Bun slides in from right
  const bunEntrance = spring({
    frame,
    fps,
    delay: 10,
    config: { damping: 200 },
    durationInFrames: 40,
  });
  const bunX = interpolate(bunEntrance, [0, 1], [880, 480]);

  // Claws wave when they meet
  const clawWave =
    frame > 45
      ? interpolate(
          Math.sin((frame - 45) * 0.2),
          [-1, 1],
          [-10, 15],
        )
      : 0;

  // Steam appears after meeting
  const steamOpacity = interpolate(frame, [50, 65], [0, 0.7], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  // Friendship sparkle when they meet
  const showSparkles = frame > 55;
  const bobY = bob(frame, 2, 0.08);

  return (
    <>
      <Crab
        x={crabX}
        y={225 + bobY}
        clawAngle={clawWave}
      />
      <Bun
        x={bunX}
        y={225 + bobY}
        steamOpacity={steamOpacity}
      />

      {/* Friendship sparkles between them */}
      {showSparkles && (
        <>
          <Sparkle x={400} y={195} delay={55} size={14} />
          <Sparkle x={385} y={210} delay={60} size={10} />
          <Sparkle x={415} y={205} delay={65} size={12} />
          <Sparkle x={395} y={185} delay={70} size={9} />
          <Sparkle x={410} y={220} delay={75} size={11} />
        </>
      )}
    </>
  );
};
