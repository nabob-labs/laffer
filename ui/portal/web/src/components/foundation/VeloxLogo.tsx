import { twMerge } from "@laffer/foundation";
import type React from "react";

type LogoProps = {
  className?: string;
};

export const VeloxLogo: React.FC<LogoProps> = ({ className }) => {
  return (
    <img
      src="/velox-logo.svg"
      alt="velox logo"
      className={twMerge(
        "rounded-full shadow-account-card select-none bg-surface-secondary-rice",
        className,
      )}
    />
  );
};
