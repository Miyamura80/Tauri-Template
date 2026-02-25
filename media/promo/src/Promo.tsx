import type React from "react";
import { AbsoluteFill, Sequence } from "remotion";
import { SceneBuild } from "./scenes/SceneBuild";
import { SceneEntrance } from "./scenes/SceneEntrance";
import { SceneFinale } from "./scenes/SceneFinale";
import { SceneTest } from "./scenes/SceneTest";
import { Background } from "./shared/Background";

export const Promo: React.FC = () => {
  return (
    <AbsoluteFill>
      <Background />

      {/* Scene 1: Entrance – frames 0-89 */}
      <Sequence from={0} durationInFrames={90} premountFor={0}>
        <SceneEntrance />
      </Sequence>

      {/* Scene 2: Build – frames 75-179 (overlaps entrance by 15) */}
      <Sequence from={75} durationInFrames={105} premountFor={15}>
        <SceneBuild />
      </Sequence>

      {/* Scene 3: Test – frames 165-254 (overlaps build by 15) */}
      <Sequence from={165} durationInFrames={90} premountFor={15}>
        <SceneTest />
      </Sequence>

      {/* Scene 4: Finale – frames 240-299 (overlaps test by 15) */}
      <Sequence from={240} durationInFrames={60} premountFor={15}>
        <SceneFinale />
      </Sequence>
    </AbsoluteFill>
  );
};
