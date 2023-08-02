import {
  authRouter,
  paymentRouter,
  cardRouter,
  chatRouter,
  accountRouter,
  contactRouter,
  userRouter,
} from './routers';
import { router } from './trpc';

export const appRouter = router({
  auth: authRouter,
  payment: paymentRouter,
  contact: contactRouter,
  user: userRouter,
  account: accountRouter,
  chat: chatRouter,
  card: cardRouter,
});

export type AppRouter = typeof appRouter;
