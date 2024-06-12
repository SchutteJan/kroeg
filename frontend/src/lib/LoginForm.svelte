<script lang="ts">
	import { onMount } from 'svelte'
	import { login } from '../api/session'

	function handleLoginResponse(response: Response) {
		if (response.ok) {
			location.href = '/'
		} else {
			// TODO: Handle responses
			alert('Login failed: ' + response.statusText)
		}
	}

	function handleSubmit(event: Event) {
		if (!(event.target instanceof HTMLFormElement)) {
			return
		}
		login(new FormData(event.target)).then(handleLoginResponse)
	}
	const isLocalhost = location.hostname === 'localhost'
	let dummyEmail = ''
	let dummyPassword = ''

	onMount(() => {
		if (isLocalhost) {
			dummyEmail = 'some.user@gmail.com'
			dummyPassword = 'somepassw0rdthatisok'
		}
	})
</script>

<form on:submit|preventDefault={handleSubmit}>
	<fieldset>
		<label>
			Email
			<input type="text" name="email" placeholder="Email" aria-label="Login" value={dummyEmail} />
		</label>
		<label>
			Password
			<input
				type="password"
				name="password"
				placeholder="Password"
				aria-label="Password"
				required
				minlength="8"
				value={dummyPassword}
			/>
		</label>
	</fieldset>
	<input type="submit" value="Login" />
</form>
