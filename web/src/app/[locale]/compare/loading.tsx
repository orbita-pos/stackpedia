export default function CompareLoading() {
  return (
    <div className="mx-auto max-w-3xl px-4 pt-8">
      <div className="animate-pulse">
        <div className="h-8 w-48 bg-neutral-200 dark:bg-neutral-800 mb-3" />
        <div className="h-4 w-64 bg-neutral-200 dark:bg-neutral-800 mb-8" />
        <div className="flex gap-4">
          <div className="h-10 flex-1 bg-neutral-200 dark:bg-neutral-800" />
          <div className="h-10 flex-1 bg-neutral-200 dark:bg-neutral-800" />
          <div className="h-10 w-24 bg-neutral-200 dark:bg-neutral-800" />
        </div>
      </div>
    </div>
  );
}
