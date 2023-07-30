import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const appRouter = router({
  login: procedure
    .input(
      z.object({
        username: z.string(),
        password: z.string(),
      }),
    )
    .query(async (opts) => {
      const url = "http://localhost:8080/api/login"
      let login = await axios.request({
        url,
        method: "POST",
        data: {
          username: `${opts.input.username}`,
          password: `${opts.input.password}`,
        },
      });

      return {
        login: login.data,
      };
    }),
  payments: procedure
    .query(async () => {
      const url = "http://localhost:8080/api/payments"
      const headers = {
        Cookie: "auth-token=user-1.exp.sign"
      }
      let payments = await axios.request({
        method: "GET",
        url,
        headers
      });

      return {
        payments: payments.data,
      };
    }),
  payment: procedure
    .input(
      z.object({
        amount: z.string(),
        sender: z.string(),
        receiver: z.string(),
        description: z.string(),
      }),
    )
    .mutation(async (opts) => {
      const url = "http://localhost:8080/api/payments"
      const headers = {
        Cookie: "auth-token=user-1.exp.sign"
      }
      let payments = await axios.request({
        method: "POST",
        url,
        data: {
          amount: `${opts.input.amount}`,
          sender: `${opts.input.sender}`,
          receiver: `${opts.input.receiver}`,
          description: `${opts.input.description}`,
        },
        headers
      });

      return {
        payments: payments.data,
      };
    }),
});

// export type definition of API
export type AppRouter = typeof appRouter;
