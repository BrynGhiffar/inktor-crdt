import { DetailedHTMLProps, Dispatch, HTMLAttributes, PropsWithChildren, SetStateAction } from "react";

export type DivProps = PropsWithChildren & DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
export type SelectedSVG = { id: string, type: "CIRCLE" | "RECTANGLE" | "PATH" | "GROUP" };
export type SelectedObject = SelectedSVG | "root";
export type DroppableSVG = ({ 
    type: "CIRCLE_TAG", 
    depth: number,
    id: string,
  }) | 
  ({ 
    type: "RECTANGLE_TAG",
    depth: number,
    id: string,
  }) | 
  ({ 
    type: "PATH_TAG",
    depth: number,
    id: string,
  })| 
  ({
    type: "GROUP_START_TAG",
    depth: number,
    id: string,
  }) |
  ({
    type: "GROUP_END_TAG",
    depth: number,
    id: string,
  })
  ;
export type ReactSetSVGObject = Dispatch<SetStateAction<SelectedObject>>;
export type ReactSVGObjectState = [SelectedObject, ReactSetSVGObject];
