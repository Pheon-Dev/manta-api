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
  withdraw: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
        amount: z.number(),
        card_id: z.string(),
        username: z.string(),
      }),
    )
    .mutation(async (opts) => {
      const get_card_method = "get_card"
      const cookie = `${opts.input.cookie}`
      const card_id = `${opts.input.card_id}`
      const username = `${opts.input.username}`
      const amount = opts.input.amount
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      let get_card_data_response = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          card_id,
          method: get_card_method,
        }
      });

      const card = get_card_data_response.data
      console.log(card)

      return {
        get_card_data_response: get_card_data_response.data,
      };
    }),
  deposit: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
        amount: z.number(),
        card_id: z.string(),
        username: z.string(),
      }),
    )
    .mutation(async (opts) => {
      const get_card_method = "get_card"
      const cookie = `${opts.input.cookie}`
      const card_id = `${opts.input.card_id}`
      const username = `${opts.input.username}`
      const amount = opts.input.amount
      const id = opts.input.id
      const url = "http://localhost:8080/api/rpc"
      const headers = {
        Cookie: cookie
      }
      let get_card_data_response = await axios.request({
        method: "POST",
        url,
        headers,
        data: {
          id: 1,
          method: get_card_method,
        params: {
          id: +card_id
        }
        }
      });

      const card = get_card_data_response.data
      console.log(card)

      return {
        get_card_data_response: get_card_data_response.data,
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
