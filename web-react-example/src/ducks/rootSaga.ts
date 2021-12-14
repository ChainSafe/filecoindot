import { all } from 'redux-saga/effects';
import { authSagaWatcher } from './substrate/sagas';

export function* rootSaga(): Generator {
  yield all([authSagaWatcher()]);
}
