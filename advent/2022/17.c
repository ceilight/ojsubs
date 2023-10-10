#include <assert.h>
#include <ctype.h>
#include <inttypes.h>
#include <stdio.h>

#define LEN(a)  (sizeof(a) / sizeof(*(a)))

#define SW      4
#define SH      4
#define GW      7
#define GH      10000
#define SPAWNX  2
#define SPAWNY  3
#define P2      1000000000000

struct state {
    int t, pi, top;
    int64_t nstack;
};

// Dot positions are relative to the bottom left corner of the shape 
static const char shapes[][SH][SW+1] = {
    { "####", "....", "....", "...." },
    { ".#..", "###.", ".#..", "...." },
    { "###.", "..#.", "..#.", "...." },
    { "#...", "#...", "#...", "#..." },
    { "##..", "##..", "....", "...." }
};

static char jets[1<<14];
static char grid[GH];
static struct state memo[4096];
static size_t nstate;

static int min(int a, int b) { return a < b ? a : b; }

// Each row in the grid can be represented as a sequence of 7 bits
static void occupy_grid_at(int px, int py)
{
    grid[py] |= (1 << px);
}

static char grid_at(int px, int py)
{
    return (grid[py] >> px) & 1;
}

// Store the given state for cycle detection later on
static void memo_push(int t, int pi, int top, int64_t nstack)
{
    assert(nstate < LEN(memo));
    memo[nstate].t = t;
    memo[nstate].pi = pi;
    memo[nstate].top = top;
    memo[nstate].nstack = nstack;
    nstate++;
}

// Do a reverse traversal of the first 'lim' of memo for maching t and pi.
static struct state *memo_find(size_t lim, int t, int pi)
{
    for (int i = lim-1; i >= 0; i--)
        if (memo[i].t == t && memo[i].pi == pi)
            return &memo[i];

    return NULL;
}

// Find a repeating game state pattern by taking the top of the memo
// stack and looking for two more memo entries with the same piece
// and jet position that are equal in distance
static int find_cycle(int *dtop, int64_t *dnstack)
{
    struct state *s1 = &memo[nstate-1], *s2, *s3;

    for (s2 = s1; s2 = memo_find(s2-memo, s1->t, s1->pi);) {
        for (s3 = s2; s3 = memo_find(s3-memo, s1->t, s1->pi);) {
            if (s1->top - s2->top == s2->top - s3->top
                && s2->nstack - s1->nstack == s3->nstack - s2->nstack)
            {
                *dtop = s2->top - s1->top;
                *dnstack = s1->nstack - s2->nstack;
                return 1;
            }
        }
    }

    return 0;
}

// Check if a piece (pi) collides with the settled pieces in grid
// supposing its bottom left corner position is (px, py)
static int check_collision(int pi, int px, int py)
{
    if (px < 0 || py < 0)
        return 1;

    for (int y = 0; y < SH; y++) {
        for (int x = 0; x < SW; x++) {
            if (shapes[pi][y][x] == '#'
                && (py+y >= GH || px+x >= GW || grid_at(px+x, py+y)))
                return 1;
        }
    }

    return 0;
}

int main(void)
{
    int nr = (int)fread(jets, 1, sizeof(jets), stdin);
    while (nr > 0 && isspace(jets[nr-1]))
        nr--;

    // Basic tetris simulation with pi/px/py representing the
    // current piece and its position, and t (for 'time') the
    // position within the repeating jet pattern
    int top = GH, pi = 0, px = SPAWNX, py = SPAWNY;
    int64_t nstack = 0, yskip = 0, p1_ans;

    for (int t = 0; nstack < P2; t = (t+1) % nr) {
        // Piece is pushed by a jet 
        int dx = jets[t] == '>' ? 1 : -1;
        if (!check_collision(pi, px+dx, py))
            px += dx;
        
        // Piece falls 1 unit, it's settled once it cannot move down
        if (!check_collision(pi, px, py-1)) {
            py--;
            continue;
        }

        // Piece is settled in the grid
        for (int y = 0; y < SH; y++) {
            for (int x = 0; x < SW; x++) {
                if (py+y < GH && px+x < GW && shapes[pi][y][x] == '#') {
                    occupy_grid_at(px+x, py+y);
                    top = min(top, GH-py-y-1);
                }
            }
        }

        if (++nstack == 2022)
            p1_ans = GH-top;

        // if detected, skip ahead by however many cycles fit between
        // nstack and the target (P2).
        //
        // The grid (and 'top') are untouched, which is OK since
        // the point of the cycle is that the game state between
        // iterations is identical. Instead, the skipped height
        // is stored in 'yskip'.' nstack' is updated as normal
        // (adding the number stacked pieces in the skip).
        if (!yskip) {
            int dtop;
            int64_t dnstack, ncycles;

            memo_push(t, pi, top, nstack);

            if (find_cycle(&dtop, &dnstack)) {
                ncycles = (P2-nstack) / dnstack;
                yskip = ncycles * dtop;
                nstack += ncycles * dnstack;
            }
        }

        pi = (pi+1) % (int)LEN(shapes);
        px = SPAWNX;
        py = GH-top+SPAWNY;
        assert(py >= 0);
    }

    printf("part 1: %"PRId64"\n", p1_ans);
    printf("part 2: %"PRId64"\n", yskip+GH-top);

    return 0;
}