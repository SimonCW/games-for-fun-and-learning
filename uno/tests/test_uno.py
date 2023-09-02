from hypothesis import given
from hypothesis.strategies import integers, lists, sampled_from, just, builds
from uno.uno import PlayerCycle

first_names = ["Alice", "Bob", "Cesar", "Zoli"]
last_names = ["Mueller", "Hu", "Gunther", "Schwarz"]

name = builds(
    lambda x, y, z: x + y + z,
    sampled_from(first_names),
    just(" "),
    sampled_from(last_names),
)
names_st = lists(elements=name, min_size=2, max_size=11)


@given(names=names_st, n_steps=integers(min_value=2, max_value=60))
def test_player_cycle_steps_forward_and_back_should_return_same_name(names, n_steps):
    """Going forward n_steps from the first player and then going the same
    number of steps backwards should yield the same player."""
    player_cycle = PlayerCycle(names)
    first_p = next(player_cycle)
    for _ in range(n_steps):
        next(player_cycle)
    player_cycle.reverse()
    for _ in range(n_steps - 1):
        next(player_cycle)
    assert next(player_cycle) == first_p
