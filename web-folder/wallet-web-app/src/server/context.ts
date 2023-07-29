import * as trpc from "@trpc/server";
import * as trpcNext from "@trpc/server/adapters/next";

interface CreateContextOptions {
  /* session: Session | null */
}

export const createContextInner = async (_opts: CreateContextOptions) => {
  return {};
};

export type Context = trpc.inferAsyncReturnType<typeof createContextInner>;

export const createContext = async (
  opts: trpcNext.CreateNextContextOptions
): Promise<Context> => {
  return await createContextInner({});
};

