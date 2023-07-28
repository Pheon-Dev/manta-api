// import React from 'react';
import { RouteObject } from 'react-router-dom';
import Wallet from '../pages/wallet';
import Dashboard from '../pages/dashboard';
import Cards from '../pages/cards';
// import Main from '@/pages/main';
import NotFound from '../pages/NotFound';

export default [
  {
    path: '/',
    element: <Wallet />,
    // children: [{ index: true, element: <Main /> }],
  },
  {
    path: '/dashboard',
    element: <Dashboard />,
    // children: [{ index: true, element: <Main /> }],
  },
  {
    path: '/cards',
    element: <Cards />,
    // children: [{ index: true, element: <Main /> }],
  },
  {
    path: '/*',
    element: <NotFound />,
  },
] as RouteObject[];
