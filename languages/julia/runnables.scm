; Julia @testset blocks
; Example: @testset "My Test Suite" begin ... end
(
  (macrocall_expression
    (macro_identifier "@" (identifier) @_macro)
    (macro_argument_list
      .
      (string_literal
        (content) @run @JULIA_TESTSET_NAME
      )
    )
  )
  (#eq? @_macro "testset")
  (#set! tag julia-testset)
)

; ; Julia @test expressions
; ; Example: @test 1 + 1 == 2
; (
;   (macrocall_expression
;     (macro_identifier "@" (identifier) @_macro)
;   ) @run @JULIA_TEST_EXPR
;   (#eq? @_macro "test")
;   (#set! tag julia-test)
; )

; Julia test runner comments
; Example:
; # [TestRunner]
; test_func(...)
(
  (line_comment) @_comment
  (#match? @_comment "^#\\s*\\[TestRunner\\]")
  .
  (_) @run @JULIA_TESTRUNNER_EXPR
  (#set! tag julia-testrunner)
)
