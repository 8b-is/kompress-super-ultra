//! End-to-end integration tests for the 4-role semantic fold engine, CLI parser, and `AppState`.

mod common;

use kompress_super_ultra::{
    cli::{Cli, Commands},
    evaluate_bonobo_resonance, AdvancedKompressEngine, FoldMode,
};

#[test]
fn e2e_four_role_semantic_fold_and_state_accounting() {
    let state = common::test_state();
    let outcome =
        state.engine.fold_with_metrics(common::SAMPLE_NOISY_PAYLOAD, FoldMode::Aggressive);

    assert!(outcome.original_tokens > outcome.compressed_tokens);
    assert!(outcome.saved_tokens > 0);
    assert!(outcome.compressed.contains("[...hex dump pruned...]"));
    assert!(outcome.compressed.contains("... [repeated 3 times] ..."));
    assert!(outcome.compressed.contains("[TS]"));
    assert!(outcome.compressed.contains("[UUID]"));
    assert!(outcome
        .compressed
        .contains("{\"ok\":true,\"pq\":\"3/2\",\"rung\":0,\"service\":\"kompress-super-ultra\"}"));

    // Record fold into AppState and verify exact p/q accounting
    let entry = state.record_fold(common::SAMPLE_NOISY_PAYLOAD.len(), &outcome).unwrap();
    assert_eq!(entry.id, 1);
    assert_eq!(entry.saved_tokens, outcome.saved_tokens);
    assert_eq!(entry.savings_bps, outcome.savings_bps);

    let stats = state.stats.lock().unwrap().clone();
    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.input_tokens, outcome.original_tokens);
    assert_eq!(stats.output_tokens, outcome.compressed_tokens);
    assert_eq!(stats.saved_tokens, outcome.saved_tokens);
    assert_eq!(stats.bonobo_harmonic_pq, "3/2");
    assert_eq!(stats.techno_zen_bonobo_pq, "9/8");
    assert_eq!(
        stats.target_resonance_trit,
        evaluate_bonobo_resonance(stats.input_tokens, stats.output_tokens)
    );
}

#[test]
fn cli_parser_handles_init_run_serve_and_bench() {
    let default_cli = Cli::parse_from(["kompress-super-ultra".to_string()]);
    assert!(matches!(default_cli.command, Commands::Serve(_)));

    let init_cli = Cli::parse_from([
        "kompress-super-ultra".to_string(),
        "init".to_string(),
        "--name".to_string(),
        "solar-octad".to_string(),
    ]);
    match init_cli.command {
        Commands::Init(args) => assert_eq!(args.name, "solar-octad"),
        other => panic!("expected Init, got {other:?}"),
    }

    let run_cli = Cli::parse_from([
        "kompress-super-ultra".to_string(),
        "run".to_string(),
        "--verbose".to_string(),
        "--mode".to_string(),
        "aggressive".to_string(),
    ]);
    match run_cli.command {
        Commands::Run(args) => {
            assert!(args.verbose);
            assert_eq!(args.mode, FoldMode::Aggressive);
        }
        other => panic!("expected Run, got {other:?}"),
    }

    let engine = AdvancedKompressEngine::new();
    let json_only =
        engine.fold_with_mode("{\n  \"a\": 1,\n  \"b\": [1, 2, 3]\n}", FoldMode::JsonOnly);
    assert_eq!(json_only, "{\"a\":1,\"b\":[1,2,3]}");
}
