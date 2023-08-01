import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;

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
  logoff: procedure
    .query(async (opts) => {
      const url = "http://localhost:8080/api/logoff"
      let logoff = await axios.request({
        url,
        method: "POST",
        data: {
          logoff: true
        },
      });

      return {
        logoff: logoff.data,
      };
    }),
  payments: procedure
    .query(async () => {
      const url = "http://localhost:8081/api/payments"
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
      const url = "http://localhost:8081/api/payments"
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
  send: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
        amount: z.string(),
        sender: z.string(),
        receiver: z.string(),
        description: z.string(),
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
              amount: `${opts.input.amount}`,
              sender: `${opts.input.sender}`,
              receiver: `${opts.input.receiver}`,
              description: `${opts.input.description}`,
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
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
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
      let payments = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          id,
          method,
        }
      });

      return {
        payments: payments.data,
      };
    }),
  chat: procedure
    .query(async () => {
      const url = "https://api.openai.com/v1/chat/completions"
      const headers = {
        "Authorization": `Bearer ${OPENAI_API_KEY}`,
        "Content-Type": "application/json",
      }
      let payments = await axios.request({
        method: "POST",
        url,
        data: {
          model: "gpt-3.5-turbo",
          messages: [
            {
              role: "system",
              content: "You are a helpful assistant."
            },
            {
              role: "user",
              content: "Hello!"
            }
          ]
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