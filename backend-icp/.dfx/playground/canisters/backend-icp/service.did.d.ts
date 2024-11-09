import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Result = { 'Ok' : string } |
  { 'Err' : string };
export interface State {
  'timer_id' : bigint,
  'logs' : Array<string>,
  'poll_count' : bigint,
}
export interface _SERVICE {
  'watch_btc_event_transfer_start' : ActorMethod<[], Result>,
  'watch_btc_proposal_event_get' : ActorMethod<[], Result>,
  'watch_btc_proposal_event_is_polling' : ActorMethod<[], Result>,
  'watch_btc_proposal_event_poll_count' : ActorMethod<[], Result>,
  'watch_btc_proposal_event_stop' : ActorMethod<[], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
