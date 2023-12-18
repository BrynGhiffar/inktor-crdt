import { DetailedHTMLProps, HTMLAttributes, PropsWithChildren } from "react";

export type DivProps = PropsWithChildren & DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
export type SelectedSVG = { id: string, type: "CIRCLE" | "RECTANGLE" | "PATH" | "GROUP" };
export type SelectedObject = SelectedSVG | "root";