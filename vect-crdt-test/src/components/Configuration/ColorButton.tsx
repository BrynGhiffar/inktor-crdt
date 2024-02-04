import { DetailedHTMLProps, FC, HTMLAttributes } from "react";

type ColorButtonProps =
& DetailedHTMLProps<HTMLAttributes<HTMLButtonElement>, HTMLButtonElement>
& {
    pickerColor: [number, number, number, number]
} 

export const ColorButton: FC<ColorButtonProps> = ({ onClick, children, pickerColor, ...props }) => {
    const [ red, green, blue, opacity ] = pickerColor;
    return (
    <div className="relative flex items-start">
        <button
            style={{ background: `rgba(${red}, ${green}, ${blue}, ${opacity})` }}
            className="w-10 h-3 cursor-pointer border border-black"
            onClick={(e) => { 
                e.stopPropagation();
                e.preventDefault(); 
                if (onClick) { onClick(e) }
            }}
            { ...props }
        />
        { children }
    </div>
    );
}