import { DetailedHTMLProps, FC, HTMLAttributes } from "react";
import { twMerge } from "tailwind-merge";

type CodeFragmentProps = {
    noRightPadding?: boolean;
    noLeftPadding?: boolean,
    noRoundLeft?: boolean;
    noRoundRight?: boolean;
    selected?: boolean;
} & DetailedHTMLProps<HTMLAttributes<HTMLElement>, HTMLElement>;

export const CodeFragment: FC<CodeFragmentProps> = ({ children, selected, noRoundLeft, noRoundRight, noLeftPadding, noRightPadding, ...other }) => {

    return <code {...other} className={twMerge(
        selected ? "bg-dark-blue" : "",
        "cursor-pointer px-1 rounded-sm text-white whitespace-nowrap",
        noRoundLeft ? "rounded-tl-none rounded-bl-none" : "",
        noRoundRight ? "rounded-tr-none rounded-br-none" : "",
        noLeftPadding ? "pl-0" : "",
        noRightPadding ? "pr-0" : ""
    )}>{children}</code>
}