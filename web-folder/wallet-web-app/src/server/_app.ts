import {
  authRouter,
  paymentRouter,
  cardRouter,
  chatRouter,
  accountRouter,
  contactRouter,
} from './routers';
import { router } from './trpc';

export const appRouter = router({
  auth: authRouter,
  payment: paymentRouter,
  contact: contactRouter,
  account: accountRouter,
  chat: chatRouter,
  card: cardRouter,
});

export type AppRouter = typeof appRouter;
