import { combineReducers, createStore, applyMiddleware, compose, Store } from 'redux';
import reduxSaga from 'redux-saga';
import { rootSaga } from './rootSaga';
import { createAction } from '@reduxjs/toolkit';
import { substrateSlice } from './substrate/slice';

// state
export const rootReducer = combineReducers({
  substrate: substrateSlice.reducer,
});
export type RootState = ReturnType<typeof rootReducer>;

// middlewares
const sagaMiddleware = reduxSaga();
const middleware = [sagaMiddleware];

// store
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
export const storeCreator = (initialState?: Partial<RootState>): Store<RootState> =>
  createStore(
    rootReducer,
    { ...(initialState || {}) },
    process.env.NODE_ENV === 'production'
      ? applyMiddleware(...middleware)
      : composeEnhancers(applyMiddleware(...middleware)),
  );
export const store = storeCreator();

// special action - sagas can hook on it to make special actions required for app (can be async)
export const postInit = createAction('@@POST_INIT');

sagaMiddleware.run(rootSaga);

store.dispatch(postInit());
