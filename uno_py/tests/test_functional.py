from uno.game import main
from uno.state import CommunityCards
import pytest


@pytest.mark.parametrize("execution_number", range(100))
def test_games_should_finish_wo_exceptions(execution_number):
    main()


def test_deck_should_have_108_cards():
    cc = CommunityCards.new()
    assert len(cc._deck) == 108
    assert len(cc._pile) == 0
