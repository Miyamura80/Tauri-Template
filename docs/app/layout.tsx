import { RootProvider } from "fumadocs-ui/provider/next";
import "./global.css";
import type { Metadata } from "next";
import { Archivo } from "next/font/google";
import type { ReactNode } from "react";

const archivo = Archivo({
	subsets: ["latin"],
	weight: ["500"],
});

export const metadata: Metadata = {
	title: "Tauri Template Documentation",
	description: "Opinionated starter for Tauri + React + Bun applications",
	icons: {
		icon: [
			{
				url: "/favicon.ico",
			},
			{
				url: "/icon-light.png",
				media: "(prefers-color-scheme: light)",
			},
			{
				url: "/icon-dark.png",
				media: "(prefers-color-scheme: dark)",
			},
		],
		apple: "/icon-light.png",
	},
};

export default function Layout({ children }: { children: ReactNode }) {
	return (
		<html lang="en" className={archivo.className} suppressHydrationWarning>
			<body className="flex flex-col min-h-screen" suppressHydrationWarning>
				<RootProvider>{children}</RootProvider>
			</body>
		</html>
	);
}
