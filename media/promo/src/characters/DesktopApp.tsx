import type React from "react";
import { interpolate } from "remotion";
import { COLORS } from "../shared/colors";

type Props = {
  x: number;
  y: number;
  titleBarScale?: number;
  bodyHeight?: number;
  contentOpacity?: number;
  borderGlow?: number;
};

export const DesktopApp: React.FC<Props> = ({
  x,
  y,
  titleBarScale = 1,
  bodyHeight = 1,
  contentOpacity = 0,
  borderGlow = 0,
}) => {
  const width = 200;
  const maxHeight = 150;
  const currentHeight = interpolate(bodyHeight, [0, 1], [0, maxHeight]);
  const titleBarH = 28 * titleBarScale;

  const glowShadow = `0 0 ${borderGlow * 20}px ${borderGlow * 8}px rgba(36,200,219,${borderGlow * 0.4})`;

  return (
    <div
      style={{
        position: "absolute",
        left: x - width / 2,
        top: y - (titleBarH + currentHeight) / 2,
        width,
        overflow: "hidden",
        borderRadius: 10,
        boxShadow: borderGlow > 0 ? glowShadow : "0 4px 20px rgba(0,0,0,0.4)",
      }}
    >
      {/* Title bar */}
      {titleBarScale > 0 && (
        <div
          style={{
            height: titleBarH,
            background: COLORS.appTitleBar,
            display: "flex",
            alignItems: "center",
            paddingLeft: 10,
            gap: 6,
          }}
        >
          <div
            style={{
              width: 10,
              height: 10,
              borderRadius: "50%",
              background: COLORS.appDotRed,
            }}
          />
          <div
            style={{
              width: 10,
              height: 10,
              borderRadius: "50%",
              background: COLORS.appDotYellow,
            }}
          />
          <div
            style={{
              width: 10,
              height: 10,
              borderRadius: "50%",
              background: COLORS.appDotGreen,
            }}
          />
          <div
            style={{
              flex: 1,
              textAlign: "center",
              color: COLORS.textSecondary,
              fontSize: 11,
              fontFamily: "monospace",
              opacity: contentOpacity,
            }}
          >
            tauri-template
          </div>
        </div>
      )}

      {/* Body */}
      {currentHeight > 0 && (
        <div
          style={{
            height: currentHeight,
            background: COLORS.appBody,
            padding: 12,
            position: "relative",
          }}
        >
          {/* Placeholder UI content */}
          <div style={{ opacity: contentOpacity }}>
            {/* Sidebar hint */}
            <div
              style={{
                position: "absolute",
                left: 12,
                top: 12,
                width: 40,
                height: currentHeight - 24,
                background: COLORS.appContent,
                borderRadius: 4,
                opacity: 0.5,
              }}
            />
            {/* Main content area */}
            <div
              style={{
                position: "absolute",
                left: 62,
                top: 12,
                right: 12,
                display: "flex",
                flexDirection: "column",
                gap: 8,
              }}
            >
              <div
                style={{
                  height: 10,
                  width: "80%",
                  background: COLORS.appContent,
                  borderRadius: 3,
                }}
              />
              <div
                style={{
                  height: 10,
                  width: "60%",
                  background: COLORS.appContent,
                  borderRadius: 3,
                }}
              />
              <div
                style={{
                  height: 10,
                  width: "70%",
                  background: COLORS.appContent,
                  borderRadius: 3,
                }}
              />
            </div>

            {/* Tauri logo (simplified T icon) */}
            <div
              style={{
                position: "absolute",
                bottom: 14,
                right: 14,
                width: 30,
                height: 30,
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <svg width={28} height={28} viewBox="0 0 28 28">
                <circle
                  cx={14}
                  cy={10}
                  r={5}
                  fill="none"
                  stroke={COLORS.tauriBlue}
                  strokeWidth={2}
                />
                <circle
                  cx={14}
                  cy={20}
                  r={4}
                  fill="none"
                  stroke={COLORS.tauriBlue}
                  strokeWidth={2}
                />
                <line
                  x1={14}
                  y1={15}
                  x2={14}
                  y2={16}
                  stroke={COLORS.tauriBlue}
                  strokeWidth={2}
                />
              </svg>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
