import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const userRouter = router({
  list: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
      }),
    )
    .query(async (opts) => {
    try {
      const method = `${opts.input.method}`
      const cookie = `${opts.input.cookie}`
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      let response = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          id,
          method,
        }
      });

      return {
        response: response.data,
      };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
  create: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
        password_clear: z.string(),
        username: z.string(),
        email: z.string(),
      }),
    )
    .mutation(async (opts) => {
    try {
      const method = `${opts.input.method}`
      const cookie = `${opts.input.cookie}`
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      
      let response = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          id,
          method,
          params: {
            data: {
              password_clear: `${opts.input.password_clear}`,
              username: `${opts.input.username}`,
              email: `${opts.input.email}`,
            }
          }
        }
      });

      return {
        response: response.data,
      };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
});


