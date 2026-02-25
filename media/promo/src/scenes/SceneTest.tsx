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
import { RobotTester } from "../characters/RobotTester";
import { bob } from "../shared/animations";
import { Checkmark } from "../shared/Checkmark";

export const SceneTest: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const bobY = bob(frame, 2, 0.1);

  // Three robots march in from below (staggered)
  const robot1Enter = spring({
    frame,
    fps,
    delay: 5,
    config: { damping: 15 },
  });
  const robot2Enter = spring({
    frame,
    fps,
    delay: 15,
    config: { damping: 15 },
  });
  const robot3Enter = spring({
    frame,
    fps,
    delay: 25,
    config: { damping: 15 },
  });

  const robot1Y = interpolate(robot1Enter, [0, 1], [500, 350]);
  const robot2Y = interpolate(robot2Enter, [0, 1], [500, 360]);
  const robot3Y = interpolate(robot3Enter, [0, 1], [500, 350]);

  // Arm raise while "scanning"
  const scanStart = 40;
  const arm1Raise = interpolate(frame, [scanStart, scanStart + 15], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const arm2Raise = interpolate(
    frame,
    [scanStart + 10, scanStart + 25],
    [0, 1],
    { extrapolateLeft: "clamp", extrapolateRight: "clamp" },
  );
  const arm3Raise = interpolate(
    frame,
    [scanStart + 20, scanStart + 35],
    [0, 1],
    { extrapolateLeft: "clamp", extrapolateRight: "clamp" },
  );

  // Antenna glow during scanning
  const glow1 = frame > scanStart ? 1 : 0;
  const glow2 = frame > scanStart + 10 ? 1 : 0;
  const glow3 = frame > scanStart + 20 ? 1 : 0;

  // Scan beam
  const scanBeamOpacity = interpolate(
    frame,
    [scanStart, scanStart + 10, scanStart + 40, scanStart + 50],
    [0, 0.3, 0.3, 0],
    { extrapolateLeft: "clamp", extrapolateRight: "clamp" },
  );

  return (
    <>
      {/* Characters stay in their build positions */}
      <Crab x={180} y={250 + bobY} clawAngle={5} scale={0.9} />
      <Bun x={300} y={250 + bobY} steamOpacity={0.3} scale={0.9} />

      {/* Desktop app (fully built) */}
      <DesktopApp
        x={550}
        y={225}
        titleBarScale={1}
        bodyHeight={1}
        contentOpacity={1}
        borderGlow={0.3}
      />

      {/* Scan beam effect */}
      {scanBeamOpacity > 0 && (
        <div
          style={{
            position: "absolute",
            left: 450,
            top: 155,
            width: 200,
            height: 140,
            border: "2px solid rgba(0, 230, 118, 0.3)",
            borderRadius: 10,
            opacity: scanBeamOpacity,
            boxShadow: "inset 0 0 20px rgba(0, 230, 118, 0.1)",
          }}
        />
      )}

      {/* Robots */}
      <RobotTester
        x={470}
        y={robot1Y}
        armRaise={arm1Raise}
        antennaGlow={glow1}
        variant={0}
      />
      <RobotTester
        x={550}
        y={robot2Y}
        armRaise={arm2Raise}
        antennaGlow={glow2}
        variant={1}
      />
      <RobotTester
        x={630}
        y={robot3Y}
        armRaise={arm3Raise}
        antennaGlow={glow3}
        variant={2}
      />

      {/* Checkmarks appear one by one after scanning */}
      <Checkmark x={470} y={330} delay={60} size={22} />
      <Checkmark x={550} y={340} delay={68} size={22} />
      <Checkmark x={630} y={330} delay={76} size={22} />
    </>
  );
};
