import { DroppableSVG, SelectedObject } from "../types";
import { SVGDocTree, SVGGroup } from "@brynghiffar/vect-crdt-rs";

export const px = (num: number) => `${num}px`;

const numToHex = (num: number) => {
    const hex = num.toString(16);
    return `${hex.length === 1 ? "0" : ""}${hex}`;
}

export const rgbToHex = (red: number, green: number, blue: number) => {
    return `#${numToHex(red)}${numToHex(green)}${numToHex(blue)}`;
}

export const saveSVGData = (docId: string, data: string) => {
    localStorage.setItem(`SVG_DATA_${docId}`, data);
};

export const getSVGData = (docId: string): string | null => {
    return localStorage.getItem(`SVG_DATA_${docId}`);
}

export const isObjectSelected = (id: string, obj: SelectedObject) => {
  if (obj === "root") return id === "root";
  return obj.id === id;
}

const flattenSVGGroup = (group: SVGGroup, depth: number): DroppableSVG[] => {
  const result: DroppableSVG[] = [];
  const childDepth = depth + 1;
  for (const child of group.children) {
    switch (child.type) {
      case "CIRCLE":
        result.push({ type: "CIRCLE_TAG", depth: childDepth, id: child.id })
        break;
      case "RECTANGLE":
        result.push({ type: "RECTANGLE_TAG", depth: childDepth, id: child.id })
        break;
      case "PATH":
        result.push({ type: "PATH_TAG", depth: childDepth, id: child.id })
        break;
      case "GROUP":
        result.push({ type: "GROUP_START_TAG", depth: childDepth, id: child.id })
        result.push(...flattenSVGGroup(child, childDepth));
        result.push({ type: "GROUP_END_TAG", depth: childDepth, id: `END_${child.id}` });
        break;
    }
  }
  return result;
}

export const flattenSVGDocTree = (tree: SVGDocTree) => {
  const result: DroppableSVG[] = []
  const depth = 1;
  for (const child of tree.children) {
    const { id } = child;
    switch (child.type) {
      case "CIRCLE":
        result.push({ type: "CIRCLE_TAG", depth, id });
        break;
      case "RECTANGLE":
        result.push({ type: "RECTANGLE_TAG", depth, id });
        break;
      case "PATH":
        result.push({ type: "PATH_TAG", depth, id });
        break;
      case "GROUP":
        result.push({ type: "GROUP_START_TAG", depth, id: `${id}` });
        result.push(...flattenSVGGroup(child, depth))
        result.push({ type: "GROUP_END_TAG", depth, id: `END_${id}` });
    }
  }
  return result;
};

export const getGroupId = (id: string, flattenTree: DroppableSVG[]): string | null => {
  // returns "root" when element not in group.
  // if null is returned that means id does not exist.
  const index = flattenTree.findIndex(v => v.id === id);
  if (index === -1) return null;
  const n = flattenTree.length;
  let i = index + 1;
  while ( i < n ) {
    const current = flattenTree[i];
    if (current.type === "GROUP_END_TAG" && current.id === `END_${id}`) {
      i++;
      continue;
    }
    if (flattenTree[i].type === "GROUP_END_TAG") {
      return flattenTree[i].id.replace("END_", "");
    }
    if (current.type === "GROUP_START_TAG") {
      i++;
      const groupId = current.id;
      while ( i < n ) {
        const next = flattenTree[i];
        if (next.type === "GROUP_END_TAG" && next.id === `END_${groupId}`) {
          i++;
          break;
        }
        i++;
      }
    } else {
      i++;
    }

  }
  return "root";
}

export const getIndexInGroup = (
  groupId: string, 
  id: string, 
  flattenTree: DroppableSVG[]
): number | null => {
  const groupIdIndex = groupId === "root" ? -1 : flattenTree.findIndex(item => item.id === groupId);
  const groupIdEndIndex = groupId === "root" ? flattenTree.length :flattenTree.findIndex(item => item.id === `END_${groupId}`);
  if (groupIdIndex === -1 && groupId !== "root") return null;
  if (groupIdEndIndex === undefined) return null;
  let itemsBehind = 0;
  let curr = groupIdIndex + 1;
  while (curr < groupIdEndIndex) {
    const current = flattenTree[curr];
    if (current.id === id.replace("END_", "")) { return itemsBehind; }
    if (current.type === "GROUP_START_TAG") {
      const next = flattenTree.findIndex(v => v.id === `END_${current.id}`);
      if (next === -1) return null;
      curr = next + 1;
    } else {
      curr++;
    }
    itemsBehind++;
  }
  return itemsBehind;
};