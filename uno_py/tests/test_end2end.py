from uno.game import main
import pytest


@pytest.mark.parametrize("execution_number", range(100))
def test_games_should_finish_wo_exceptions(execution_number):
    main()
