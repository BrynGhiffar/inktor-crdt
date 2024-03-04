import { FC, CSSProperties, forwardRef } from 'react';
import { DivProps } from '../types';

export const TwoColumnGrid: FC<DivProps> = (props) => {
    const { children, style: propsStyle, ...otherProps } = props;
    const defaultStyle: CSSProperties = { display: "grid", gridTemplateColumns: "1fr 1fr" };
    const style = { ...defaultStyle, ...propsStyle };
    return (
        <div style={style} { ...otherProps }>{ children }</div>
    )
}

export const NColumnGrid: FC<DivProps & { nColumns: number }> = (props) => {
    const { children, style: propsStyle, nColumns, ...otherProps } = props;
    const defaultStyle: CSSProperties = { display: "grid", gridTemplateColumns: `repeat(${nColumns}, 1fr)` };
    const style = { ...defaultStyle, ...propsStyle };
    return (
        <div style={style} {...otherProps}>{children}</div>
    )
}

type PaddedDivProps = DivProps & {
  depth?: number,
}

export const PaddedDiv = forwardRef<HTMLDivElement, PaddedDivProps>((props, ref) => {
  const { depth, children, style: propsStyle, ...divProps } = props;
  const style = { paddingLeft: `${20 * (depth ?? 1)}px`, ...propsStyle };
  return (
    <div
      ref={ref}
      {...divProps}
      style={style}
    >{children}</div>
  )
});
