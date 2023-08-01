import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const accountRouter = router({
  create: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
        username: z.string(),
        balance: z.string(),
        email: z.string(),
        aid: z.string(),
      }),
    )
    .mutation(async (opts) => {
      const method = `${opts.input.method}`
      const cookie = `${opts.input.cookie}`
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      let payments = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          id,
          method,
          params: {
            data: {
              username: `${opts.input.username}`,
              balance: `${opts.input.balance}`,
              email: `${opts.input.email}`,
              aid: `${opts.input.aid}`,
            }
          }
        }
      });

      return {
        payments: payments.data,
      };
    }),
  list: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
      }),
    )
    .query(async (opts) => {
      const method = `${opts.input.method}`
      const cookie = `${opts.input.cookie}`
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      try {

        let data = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id,
            method,
          }
        });

        return {
          data: data.data,
        };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error",
          solution: "Login to Server or Check your internet connection"
        }

      }
    }),
});
