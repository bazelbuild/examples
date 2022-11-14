package com.example.android.bazel

import com.google.common.truth.Truth
import org.junit.Test

class AndroidHelperTest {

    @Test
    fun `one plus one equals two`() {
        Truth.assertThat(AndroidHelper.add(1, 1))
            .isEqualTo(2)
    }

    @Test
    fun `two plus two equals four`() {
        Truth.assertThat(AndroidHelper.add(2, 2))
            .isEqualTo(4)
    }
}
