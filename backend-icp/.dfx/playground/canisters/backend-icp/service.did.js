export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  return IDL.Service({
    'watch_btc_event_transfer_start' : IDL.Func([], [Result], []),
    'watch_btc_proposal_event_get' : IDL.Func([], [Result], []),
    'watch_btc_proposal_event_is_polling' : IDL.Func([], [Result], []),
    'watch_btc_proposal_event_poll_count' : IDL.Func([], [Result], []),
    'watch_btc_proposal_event_stop' : IDL.Func([], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
