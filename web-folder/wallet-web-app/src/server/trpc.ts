import { Context } from "./context";
import { initTRPC } from "@trpc/server";
import superjson from "superjson"

// Avoid exporting the entire t-object
// since it's not very descriptive.
// For instance, the use of a t variable
// is common in i18n libraries.
// Base router and procedure helpers
export const t = initTRPC.context<Context>().create({
  transformer: superjson,
  errorFormatter({ shape }) {
    return shape;
  }
})
export const router = t.router;
export const procedure = t.procedure;
