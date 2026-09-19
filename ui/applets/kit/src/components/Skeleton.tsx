import type React from "react";
import { twMerge } from "@laffer/foundation";

export const Skeleton: React.FC<{ className?: string }> = ({ className }) => {
  return <div className={twMerge("animate-pulse bg-rice-100/50 rounded-xs", className)} />;
};
