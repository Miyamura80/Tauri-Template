import { Composition } from "remotion";
import { Promo } from "./Promo";

export const Root: React.FC = () => {
  return (
    <Composition
      id="Promo"
      component={Promo}
      durationInFrames={300}
      fps={30}
      width={800}
      height={450}
    />
  );
};
