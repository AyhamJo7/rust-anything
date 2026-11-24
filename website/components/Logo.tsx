export default function Logo({ size = 32, text = true }: { size?: number; text?: boolean }) {
  return (
    <span className="inline-flex items-center gap-2.5">
      <img
        src="/logo-mark.png"
        width={size}
        height={size}
        alt={text ? '' : 'rust-anything'}
        className="shrink-0"
      />
      {text && <span className="font-display font-semibold tracking-tight text-fg">rust-anything</span>}
    </span>
  )
}
